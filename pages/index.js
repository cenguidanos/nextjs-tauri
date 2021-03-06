import { useState } from "react";
import * as dialog from "tauri/api/dialog";
import * as window from "tauri/api/window";
import * as tauri from "tauri/api/tauri";

export default function Home() {
  const [remote, setRemote] = useState();

  const printOnRust = () => {
    tauri.invoke({
      cmd: "printOnRustSync",
      payload: "Hola desde Javascript",
    });
  };

  const getRandom = async (type) => {
    const { value, message } = await tauri.promisified({
      cmd: "generateRandomAsync",
      payload: type,
    });

    alert(
      JSON.stringify({ value, message, toInt: parseInt(value, 10) }, null, 2)
    );
  };

  const openDialog = async () => {
    const options = {
      defaultPath: "/home",
      multiple: true,
      directory: true,
    };

    const file = await dialog.open(options);
    alert(file);
  };

  const saveDialog = async () => {
    const options = {
      defaultPath: "/home",
      multiple: true,
      directory: true,
    };

    const file = await dialog.save(options);
    alert(file);
  };

  const openWindow = () => {
    window.open("https://google.com");
  };

  const remoteData = async () => {
    const request = await fetch("https://jsonplaceholder.typicode.com/users");

    const data = await request.json();
    setRemote(JSON.stringify(data, null, 2));
  };

  return (
    <div style={{ margin: "100px" }}>
      <div>
        <button onClick={openDialog}>Open dialog on Tauri</button>
        <button onClick={saveDialog}>Save dialog on Tauri</button>
        <button onClick={openWindow}>Open window on Tauri</button>
        <button onClick={remoteData}>Remote data on Tauri</button>
        <button onClick={printOnRust}>Print on Tauri (sync)</button>
        <button onClick={() => getRandom("u8")}>
          Random u8 on Tauri (async)
        </button>
        <button onClick={() => getRandom("u16")}>
          Random u16on Tauri (async)
        </button>
        <button onClick={() => getRandom("u32")}>
          Random u32 on Tauri (async)
        </button>
        <button onClick={() => getRandom("u64")}>
          Random u64 on Tauri (async)
        </button>
        <button onClick={() => getRandom("u128")}>
          Random u128 on Tauri (async)
        </button>
      </div>
      <div>
        <pre>
          <code>{remote}</code>
        </pre>
      </div>
    </div>
  );
}
