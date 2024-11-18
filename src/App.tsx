import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
    const [name, setName] = useState("");

    const generateIdentity = async () => {
        console.log(await invoke("generate_identity"));
    };

    const requestBallot = async () => {
        console.log(await invoke("request_ballot"));
    };

    const submitBallot = async () => {
        console.log(await invoke("submit_ballot", { name }));
    };

    return (
        <main className="container">
            <h1>Election Interface</h1>
            <br />
            <button onClick={generateIdentity}>Generate Identity</button>
            <br />
            <button onClick={requestBallot}>Request Ballot</button>
            <br />
            <input
                type="text"
                value={name}
                onChange={(e) => setName(e.target.value)}
                placeholder="Enter your name"
            />
            <br />
            <button onClick={submitBallot}>Vote</button>
        </main>
    );
}

export default App;
