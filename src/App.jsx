import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { basename, dirname, join } from "@tauri-apps/api/path";
import markImage from "./assets/mark.png";
import "./App.scss";

function App() {
  const [inputFiles, setInputFiles] = useState([]);
  const [outputFormat, setOutputFormat] = useState("jpg");
  const [isConverting, setIsConverting] = useState(false);
  const [convertingIndex, setConvertingIndex] = useState(-1);
  const [error, setError] = useState("");
  const [previewImage, setPreviewImage] = useState(null);
  const [isLoadingPreview, setIsLoadingPreview] = useState(false);
  const [showToast, setShowToast] = useState(false);
  const [convertedCount, setConvertedCount] = useState(0);
  const [lastConvertedCount, setLastConvertedCount] = useState(0);
  const [showPreviewModal, setShowPreviewModal] = useState(false);
  const [previewFileIndex, setPreviewFileIndex] = useState(-1);

  async function selectFiles() {
    try {
      const selected = await open({
        multiple: true,
        filters: [
          {
            name: "HEIC",
            extensions: ["heic", "heif"],
          },
        ],
      });

      if (selected) {
        const files = Array.isArray(selected) ? selected : [selected];
        
        // 파일 개수 제한 (DoS 방지)
        const MAX_FILES = 100;
        if (files.length > MAX_FILES) {
          setError(`You can select up to ${MAX_FILES} files at once.`);
          return;
        }
        
        const fileList = await Promise.all(
          files.map(async (file) => {
            const path = typeof file === "string" ? file : file.path || file;
            
            // 경로 검증 (XSS 방지)
            if (!path || typeof path !== "string") {
              throw new Error("Invalid file path.");
            }
            
            const fileName = typeof file === "string" 
              ? await basename(file)
              : file.name || (await basename(file));
            
            // 파일명 검증
            if (!fileName || fileName.length > 255) {
              throw new Error("Invalid filename.");
            }
            
            return { path, fileName };
          })
        );
        
        setInputFiles((prev) => {
          const total = prev.length + fileList.length;
          if (total > MAX_FILES) {
            setError(`Total number of files exceeds ${MAX_FILES}.`);
            return prev;
          }
          return [...prev, ...fileList];
        });
        setError("");
      }
    } catch (err) {
      // 에러 메시지에서 민감한 정보 제거
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(`File selection error: ${errorMessage.replace(/[^\w\s.,!?]/g, "")}`);
    }
  }

  function removeFile(index) {
    const newFiles = inputFiles.filter((_, i) => i !== index);
    setInputFiles(newFiles);
    
    // 미리보기 모달이 열려있고 삭제된 파일이면 모달 닫기
    if (showPreviewModal && previewFileIndex === index) {
      setShowPreviewModal(false);
      setPreviewImage(null);
    }
  }

  function clearAllFiles() {
    setInputFiles([]);
    setPreviewImage(null);
    setShowPreviewModal(false);
  }

  async function showPreview(index) {
    if (index < 0 || index >= inputFiles.length) {
      return;
    }
    
    setPreviewFileIndex(index);
    setShowPreviewModal(true);
    setPreviewImage(null);
    await loadPreviewImage(inputFiles[index].path);
  }

  function closePreviewModal() {
    setShowPreviewModal(false);
    setPreviewImage(null);
    setPreviewFileIndex(-1);
  }

  async function loadPreviewImage(file) {
    setIsLoadingPreview(true);
    try {
      const inputPath = typeof file === "string" ? file : file.path || file;
      const previewDataUrl = await invoke("get_preview_image", {
        inputPath: inputPath,
      });
      setPreviewImage(previewDataUrl);
    } catch (err) {
      console.error("Preview load error:", err);
      // 미리보기 로드 실패해도 변환은 가능하도록 에러는 표시하지 않음
    } finally {
      setIsLoadingPreview(false);
    }
  }

  async function convertAllFiles() {
    if (inputFiles.length === 0) {
      setError("Please select HEIC files first.");
      return;
    }

    setIsConverting(true);
    setError("");
    setConvertedCount(0);
    let successCount = 0;
    let failCount = 0;

    for (let i = 0; i < inputFiles.length; i++) {
      setConvertingIndex(i);
      const file = inputFiles[i];
      
      try {
        const inputPath = file.path;
        const inputDir = await dirname(inputPath);
        const inputBaseName = await basename(inputPath);
        const nameWithoutExt = inputBaseName.replace(/\.(heic|heif)$/i, "");
        const extension = outputFormat === "jpg" ? "jpg" : "png";
        const outputPath = await join(inputDir, `${nameWithoutExt}.${extension}`);

        await invoke("convert_heic_to_image", {
          inputPath: inputPath,
          outputPath: outputPath,
          format: outputFormat,
        });

        successCount++;
        setConvertedCount(successCount);
      } catch (err) {
        failCount++;
        console.error(`File conversion failed: ${file.fileName}`, err);
      }
    }

    setConvertingIndex(-1);
    setIsConverting(false);

    // 변환 완료 토스트 메시지 표시
    if (successCount > 0) {
      setLastConvertedCount(successCount);
      setShowToast(true);
      setInputFiles([]);
      setPreviewImage(null);
      setConvertedCount(0);
      setShowPreviewModal(false);
      
      if (failCount > 0) {
        setError(`${successCount} succeeded, ${failCount} failed`);
      }
    } else {
      setError("All file conversions failed.");
    }
  }

  // 토스트 메시지 자동 사라지기
  useEffect(() => {
    if (showToast) {
      const timer = setTimeout(() => {
        setShowToast(false);
      }, 3000); // 3초 후 자동 사라짐

      return () => clearTimeout(timer);
    }
  }, [showToast]);

  return (
    <main className="container">
      <div className="converter-box">
        <div className="file-selector">
          <button
            type="button"
            onClick={selectFiles}
            disabled={isConverting}
            className="select-button"
          >
            {inputFiles.length > 0 
              ? `${inputFiles.length} file(s) selected` 
              : "Select HEIC Files"}
          </button>
        </div>

        {inputFiles.length > 0 && (
          <div className="file-list">
            <div className="file-list-header">
              <span>Selected Files ({inputFiles.length})</span>
              <button
                type="button"
                onClick={clearAllFiles}
                disabled={isConverting}
                className="clear-all-button"
              >
                Clear All
              </button>
            </div>
            <div className="file-list-items">
              {inputFiles.map((file, index) => (
                <div
                  key={index}
                  className={`file-item ${convertingIndex === index ? "converting" : ""}`}
                >
                  <span className="file-name">{file.fileName}</span>
                  {convertingIndex === index && (
                    <span className="converting-badge">Converting...</span>
                  )}
                  {convertingIndex === index && convertedCount > 0 && (
                    <span className="progress-text">
                      ({convertedCount}/{inputFiles.length})
                    </span>
                  )}
                  <div className="file-item-actions">
                    <button
                      type="button"
                      onClick={() => showPreview(index)}
                      disabled={isConverting}
                      className="preview-file-button"
                      title="Preview"
                    >
                      Preview
                    </button>
                    <button
                      type="button"
                      onClick={() => removeFile(index)}
                      disabled={isConverting}
                      className="remove-file-button"
                      title="Remove"
                    >
                      ×
                    </button>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}


        <div className="format-selector">
          <label>
            <input
              type="radio"
              value="jpg"
              checked={outputFormat === "jpg"}
              onChange={(e) => {
                // 입력 검증 (XSS 방지)
                const value = e.target.value;
                if (value === "jpg" || value === "png") {
                  setOutputFormat(value);
                }
              }}
              disabled={isConverting}
            />
            JPG
          </label>
          <label>
            <input
              type="radio"
              value="png"
              checked={outputFormat === "png"}
              onChange={(e) => {
                // 입력 검증 (XSS 방지)
                const value = e.target.value;
                if (value === "jpg" || value === "png") {
                  setOutputFormat(value);
                }
              }}
              disabled={isConverting}
            />
            PNG
          </label>
        </div>

        <button
          type="button"
          onClick={convertAllFiles}
          disabled={inputFiles.length === 0 || isConverting}
          className="convert-button"
        >
          {isConverting 
            ? `Converting... (${convertedCount}/${inputFiles.length})` 
            : inputFiles.length > 0 
              ? `Convert All (${inputFiles.length})`
              : "Convert"}
        </button>

        {error && <div className="message error">{error}</div>}
      </div>

      {/* 미리보기 모달 */}
      {showPreviewModal && (
        <div className="preview-modal-overlay" onClick={closePreviewModal}>
          <div className="preview-modal" onClick={(e) => e.stopPropagation()}>
            <div className="preview-modal-header">
              <span className="preview-modal-title">
                {previewFileIndex >= 0 && inputFiles[previewFileIndex] 
                  ? inputFiles[previewFileIndex].fileName 
                  : "Preview"}
              </span>
              <button
                type="button"
                onClick={closePreviewModal}
                className="preview-modal-close"
                title="Close"
              >
                ×
              </button>
            </div>
            <div className="preview-modal-content">
              {isLoadingPreview && (
                <div className="preview-loading">
                  <div className="spinner"></div>
                  <span>Loading preview...</span>
                </div>
              )}
              {previewImage && !isLoadingPreview && (
                <div className="preview-container">
                  <img src={previewImage} alt="Preview" className="preview-image" />
                </div>
              )}
            </div>
          </div>
        </div>
      )}

      {/* 토스트 메시지 */}
      {showToast && (
        <div className="toast success">
          <div className="toast-icon">✓</div>
          <div className="toast-content">
            <div className="toast-title">Conversion Complete!</div>
            <div className="toast-message">
              {lastConvertedCount > 0 
                ? `${lastConvertedCount} image(s) have been successfully converted and saved.`
                : "Image has been successfully converted and saved."}
            </div>
          </div>
        </div>
      )}

      <div className="bottom-right-info">
        <img src={markImage} alt="Mark" className="mark-logo" />
        <span className="version-text">version 1.0.0</span>
      </div>
    </main>
  );
}

export default App;
