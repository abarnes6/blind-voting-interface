import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
    const [name, setName] = useState("");
    const [election, setElection] = useState("");
    const [elections, setElections] = useState([]);
    const [candidates, setCandidates] = useState([]);
    const [ballot, setBallot] = useState(false);

    const generateIdentity = async () => {
        console.log(await invoke("generate_identity"));
    };

    const getElections = async () => {
        setElections(await invoke("get_elections"));
    };

    const getCandidates = async () => {
        if (!election) {
            console.error("Election not set");
            return;
        }
        setCandidates(await invoke("get_candidates", { election }));
    };

    const requestBallot = async () => {
        if (await invoke("request_ballot")) {
            setBallot(true);
        }
    };

    const submitBallot = async () => {
        if (!name) {
            console.error("Name not set");
            return;
        }
        if (!ballot) {
            console.error("Ballot not requested");
            return;
        }
        if (!election) {
            console.error("Election not set");
            return;
        }
        console.log(await invoke("submit_ballot", { election, name }));
    };

    const handleChangeElection = () => {
        setElection(election);
    };

    useEffect(() => {
        generateIdentity();
    }, []);

    useEffect(() => {
        getCandidates();
    }, [election]);

    return (
        <main className="container">
            <h1>Election Interface</h1>
            <br />
            <Button onClick={getElections}>Get Elections</Button>
            <br />
            <Form.Select onChange={handleChangeElection} className="mt-3">
                <option>Select Election</option>
                {elections.map((election) => (
                    <option key={election}>{election}</option>
                ))}
            </Form.Select>
            <br />
            <Button onClick={requestBallot}>Request Ballot</Button>
            <br />
            {candidates}
            <input
                type="text"
                value={name}
                onChange={(e) => setName(e.target.value)}
                placeholder="Enter your name"
            />
            <br />
            <Button onClick={submitBallot}>Vote</Button>
        </main>
    );
}

export default App;
