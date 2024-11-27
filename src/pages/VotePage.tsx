import {
    Button,
    Container,
    Typography,
    Select,
    MenuItem,
    Stack,
} from "@mui/material";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export const VotePage = () => {
    const [election, setElection] = useState("");
    const [elections, setElections] = useState([]);
    const [candidates, setCandidates] = useState([]);
    const [candidate, setCandidate] = useState("");

    const getCandidates = async () => {
        if (!election) {
            console.error("Election not set");
            return;
        }
        setCandidates(await invoke("get_candidates", { election }));
    };

    const submitBallot = async () => {
        if (!candidate) {
            console.error("Name not set");
            return;
        }
        if (!election) {
            console.error("Election not set");
            return;
        }
        console.log(await invoke("submit_ballot", { election, candidate }));
    };

    useEffect(() => {
        const fetchData = async () => {
            setElections(await invoke("get_elections"));
        };
        fetchData();
    }, []);

    useEffect(() => {
        getCandidates();
    }, [election]);
    return (
        <Container maxWidth="sm">
            <Typography variant="h4" gutterBottom>
                Election Interface
            </Typography>
            <Stack spacing={2}>
                <Select
                    label="Select an election to vote in"
                    value={election}
                    onChange={(e) => setElection(e.target.value as string)}
                    fullWidth
                >
                    {elections.map((el) => (
                        <MenuItem key={el} value={el}>
                            {el}
                        </MenuItem>
                    ))}
                </Select>
                <Select
                    label="Select a candidate to vote for"
                    value={candidate}
                    onChange={(e) => setCandidate(e.target.value as string)}
                    fullWidth
                >
                    {candidates.map((cand) => (
                        <MenuItem key={cand} value={cand}>
                            {cand}
                        </MenuItem>
                    ))}
                </Select>
                <Button fullWidth onClick={submitBallot} variant="contained">
                    Vote
                </Button>
            </Stack>
        </Container>
    );
};
