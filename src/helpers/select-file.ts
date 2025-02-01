import { open } from "@tauri-apps/plugin-dialog";

export const selectFile = async () => {
  try {
    const filePath = await open({
      multiple: false,
      filters: [
        {
          name: "Excel Files",
          extensions: ["xls", "xlsx"],
        },
      ],
    });

    if (filePath) {
      return filePath;
    }
  } catch (error) {
    console.error("Ошибка выбора файла:", error);
  }
};
