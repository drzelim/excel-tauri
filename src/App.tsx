import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useRef, useState } from "react";
import { selectFile } from "./helpers/select-file";

const ERROR_MESSAGE = "Incorrect file";

function App() {
  const [outputPath, setOutputPath] = useState("");
  const [loading, setLoading] = useState(false);
  const timer = useRef<ReturnType<typeof setTimeout> | null>(null);

  async function start(targetPath?: string) {
    setLoading(true);
    if (timer.current) clearTimeout(timer.current);
    setOutputPath("");

    const response = await invoke("start", { targetPath });
    console.log(response);
    setLoading(false);
    setOutputPath(String(response));
    
    // timer.current = setTimeout(() => setOutputPath(""), 7500);
  }

  return (
    <main className="container">
      <h1>Группировка данных</h1>

      <p>Выберите файл для создания отчета</p>

      <div className="row">
        <button
          onClick={async (e) => {
            e.preventDefault();
            const filePath = await selectFile();
            if (!filePath) return;

            start(filePath);
          }}
        >
          Выбрать файл
        </button>
      </div>

      {
        loading && <div className="loading">Идет формирование отчета...</div>
      }

      {outputPath &&
        (outputPath === ERROR_MESSAGE ? (
          <div className="result error">Неверный формат файла</div>
        ) : (
          <div className="result"> Отчет создан - {outputPath}</div>
        ))}
    </main>
  );
}

export default App;
