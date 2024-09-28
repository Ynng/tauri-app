import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { listen } from "@tauri-apps/api/event";

function App() {
  const [greetMsg, setGreetMsg] = useState("");

  async function greet() {
    setGreetMsg(await invoke("greet", { name: "hello world" }));
  }

  useEffect(() => {
    const unlisten = listen("greet", (event) => {
      alert(`Received a greet event with payload: ${event.payload}`);
    });

    return () => {
      unlisten.then((ul) => ul());
    };
  }, []);

  return (
    <div className="container">
      <button onClick={greet}>Greet</button>

      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
