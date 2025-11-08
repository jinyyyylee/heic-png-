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
        const fileList = await Promise.all(
          files.map(async (file) => {
            const path = typeof file === "string" ? file : file.path || file;
            const fileName = typeof file === "string" 
              ? await basename(file)
              : file.name || (await basename(file));
            return { path, fileName };
          })
        );
        
        setInputFiles((prev) => [...prev, ...fileList]);
        setError("");
      }
    } catch (err) {
      setError(`파일 선택 오류: ${err}`);
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
      console.error("미리보기 로드 오류:", err);
      // 미리보기 로드 실패해도 변환은 가능하도록 에러는 표시하지 않음
    } finally {
      setIsLoadingPreview(false);
    }
  }

  async function convertAllFiles() {
    if (inputFiles.length === 0) {
      setError("먼저 HEIC 파일을 선택해주세요.");
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
        console.error(`파일 변환 실패: ${file.fileName}`, err);
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
        setError(`${successCount}개 성공, ${failCount}개 실패`);
      }
    } else {
      setError("모든 파일 변환에 실패했습니다.");
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
              ? `${inputFiles.length}개 파일 선택됨` 
              : "HEIC 파일 선택"}
          </button>
        </div>

        {inputFiles.length > 0 && (
          <div className="file-list">
            <div className="file-list-header">
              <span>선택된 파일 ({inputFiles.length}개)</span>
              <button
                type="button"
                onClick={clearAllFiles}
                disabled={isConverting}
                className="clear-all-button"
              >
                모두 지우기
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
                    <span className="converting-badge">변환 중...</span>
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
                      title="미리보기"
                    >
                      미리보기
                    </button>
                    <button
                      type="button"
                      onClick={() => removeFile(index)}
                      disabled={isConverting}
                      className="remove-file-button"
                      title="삭제"
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
              onChange={(e) => setOutputFormat(e.target.value)}
              disabled={isConverting}
            />
            JPG
          </label>
          <label>
            <input
              type="radio"
              value="png"
              checked={outputFormat === "png"}
              onChange={(e) => setOutputFormat(e.target.value)}
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
            ? `변환 중... (${convertedCount}/${inputFiles.length})` 
            : inputFiles.length > 0 
              ? `${inputFiles.length}개 모두 변환하기`
              : "변환하기"}
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
                  : "미리보기"}
              </span>
              <button
                type="button"
                onClick={closePreviewModal}
                className="preview-modal-close"
                title="닫기"
              >
                ×
              </button>
            </div>
            <div className="preview-modal-content">
              {isLoadingPreview && (
                <div className="preview-loading">
                  <div className="spinner"></div>
                  <span>미리보기 로딩 중...</span>
                </div>
              )}
              {previewImage && !isLoadingPreview && (
                <div className="preview-container">
                  <img src={previewImage} alt="미리보기" className="preview-image" />
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
            <div className="toast-title">변환 완료!</div>
            <div className="toast-message">
              {lastConvertedCount > 0 
                ? `${lastConvertedCount}개 이미지가 성공적으로 변환되어 저장되었습니다.`
                : "이미지가 성공적으로 변환되어 저장되었습니다."}
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
