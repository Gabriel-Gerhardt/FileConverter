import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Converter from "./Components/Button";
import Input from "./Components/Input_archive";
import Top_container from "./Components/Top_container";
function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="container">
      <div className="top_container">
        <Top_container title={"WizardOfFiles"}></Top_container></div>
      <div><Input></Input></div>
      <h1> </h1>
      <div><label htmlFor="">Arquivo Convertido:</label></div>
      <div className="convert">
        <Converter name={"Transform"}></Converter>
      </div>
      </main>
  );
}

export default App;
