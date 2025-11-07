import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { basename, dirname, join } from "@tauri-apps/api/path";
import markImage from "./assets/mark.png";
import "./App.scss";

function App() {
  const [inputFile, setInputFile] = useState(null);
  const [inputFileName, setInputFileName] = useState("");
  const [outputFormat, setOutputFormat] = useState("jpg");
  const [isConverting, setIsConverting] = useState(false);
  const [message, setMessage] = useState("");
  const [error, setError] = useState("");

  async function selectFile() {
    try {
      const selected = await open({
        multiple: false,
        filters: [
          {
            name: "HEIC",
            extensions: ["heic", "heif"],
          },
        ],
      });

      if (selected) {
        setInputFile(selected);
        const fileName = typeof selected === "string" 
          ? await basename(selected)
          : selected.name || (await basename(selected));
        setInputFileName(fileName);
        setMessage("");
        setError("");
      }
    } catch (err) {
      setError(`파일 선택 오류: ${err}`);
    }
  }

  async function convertFile() {
    if (!inputFile) {
      setError("먼저 HEIC 파일을 선택해주세요.");
      return;
    }

    setIsConverting(true);
    setMessage("");
    setError("");

    try {
      const inputPath = typeof inputFile === "string" ? inputFile : inputFile.path || inputFile;
      const inputDir = await dirname(inputPath);
      const inputBaseName = await basename(inputPath);
      const nameWithoutExt = inputBaseName.replace(/\.(heic|heif)$/i, "");
      const extension = outputFormat === "jpg" ? "jpg" : "png";
      const outputPath = await join(inputDir, `${nameWithoutExt}.${extension}`);

      const result = await invoke("convert_heic_to_image", {
        inputPath: inputPath,
        outputPath: outputPath,
        format: outputFormat,
      });

      setMessage(result);
      setInputFile(null);
    } catch (err) {
      const errorMsg = String(err);
      // HEIF 확장 필요 메시지인 경우 특별 처리
      if (errorMsg.includes("HEIF 이미지 확장")) {
        setError(errorMsg);
      } else {
        setError(`변환 오류: ${err}`);
      }
    } finally {
      setIsConverting(false);
    }
  }

  return (
    <main className="container">
      <h1>HEIC → JPG/PNG 변환기</h1>

      <div className="converter-box">
        <div className="file-selector">
          <button
            type="button"
            onClick={selectFile}
            disabled={isConverting}
            className="select-button"
          >
            {inputFile ? `선택됨: ${inputFileName}` : "HEIC 파일 선택"}
          </button>
        </div>

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
          onClick={convertFile}
          disabled={!inputFile || isConverting}
          className="convert-button"
        >
          {isConverting ? "변환 중..." : "변환하기"}
        </button>

        {message && <div className="message success">{message}</div>}
        {error && <div className="message error">{error}</div>}
      </div>
      <div className="bottom-right-info">
        <img src={markImage} alt="Mark" className="mark-logo" />
        <span className="version-text">version 1.0.0</span>
      </div>
    </main>
  );
}

export default App;
