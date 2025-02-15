import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useRef, useState } from "react";
import { selectFile } from "./helpers/select-file";
import { useGetVerion } from "./hooks/useGetVerion";

const ERROR_MESSAGE = "Incorrect file";

function App() {
  const [outputMsg, setOutputMsg] = useState("");
  const [loading, setLoading] = useState(false);

  const version = useGetVerion();

  const timer = useRef<ReturnType<typeof setTimeout> | null>(null);

  async function start(targetPath?: string) {
    setLoading(true);
    if (timer.current) clearTimeout(timer.current);
    setOutputMsg("");

    const response = await invoke("start", { targetPath });
    setLoading(false);
    setOutputMsg(String(response));
  }

  return (
    <main className="container">
      <h1>Группировка данных</h1>
      <p>Версия {version}</p>
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

      {loading && <div className="loading">Идет формирование отчета...</div>}

      {outputMsg &&
        (outputMsg === ERROR_MESSAGE ? (
          <div className="result error">Неверный формат файла</div>
        ) : (
          <div className="result">{outputMsg}</div>
        ))}
    </main>
  );
}

export default App;
