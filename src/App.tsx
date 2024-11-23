import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "@mantine/core/styles.css";
import {
    Button,
    Container,
    MantineProvider,
    Space,
    Title,
} from "@mantine/core";
import { Select } from "@mantine/core";

function App() {
    const [election, setElection] = useState("");
    const [elections, setElections] = useState([]);
    const [candidates, setCandidates] = useState([]);
    const [candidate, setCandidate] = useState("");
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
        if (!candidate) {
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
        console.log(await invoke("submit_ballot", { election, candidate }));
    };

    useEffect(() => {
        generateIdentity();
    }, []);

    useEffect(() => {
        getCandidates();
    }, [election]);

    return (
        <MantineProvider>
            <Container size="sm">
                <Title order={1} mb="xl">
                    Election Interface
                </Title>
                <Space h="md" />
                <Button fullWidth onClick={getElections}>
                    Get Elections
                </Button>
                <Space h="md" />
                <Select
                    label="Select an election to vote in"
                    data={elections}
                    onChange={(value) => setElection(value ?? "")}
                    placeholder="Choose election"
                />
                <Space h="md" />
                <Select
                    label="Select a candidate to vote for"
                    data={candidates}
                    onChange={(value) => setCandidate(value ?? "")}
                    placeholder="Choose candidate"
                />
                <Space h="md" />
                <Button fullWidth onClick={requestBallot}>
                    Request Ballot
                </Button>
                <Space h="md" />
                <Button fullWidth onClick={submitBallot}>
                    Vote
                </Button>
            </Container>
        </MantineProvider>
    );
}

export default App;
