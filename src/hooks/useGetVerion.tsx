import { useEffect, useState } from "react";
import { app } from "@tauri-apps/api";

export const useGetVerion = () => {
      const [version, setVersion] = useState('');

      useEffect(() => {
        async function fetchVersion() {
          const ver = await app.getVersion();
          setVersion(ver)
        }
        fetchVersion();
      }, []);
    
      return version;
}