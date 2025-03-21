import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Converter from "./Components/Button";
import Input from "./Components/Input_archive";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="container">
      <h1>WizardOfFiles</h1>
      <h3></h3>
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
