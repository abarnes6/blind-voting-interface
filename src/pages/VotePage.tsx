import {
    Button,
    Container,
    Typography,
    Select,
    MenuItem,
    Stack,
    Dialog,
    DialogTitle,
    DialogContent,
    DialogActions,
} from "@mui/material";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface Election {
    id: string;
    candidates: string[];
    vote_counts: number[];
}

export const VotePage = () => {
    const [election, setElection] = useState<Election | undefined>(undefined);
    const [elections, setElections] = useState<Election[]>([]);
    const [candidate, setCandidate] = useState<string>("");
    const [error, setError] = useState<string | null>(null);

    const fetchData = async () => {
        setElections(await invoke("get_elections"));
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
        await invoke("submit_vote", {
            election: election.id,
            candidateIndex: election.candidates.indexOf(candidate),
        }).catch((e) => {
            setError(e);
        });
        await fetchData();
        setElection(election);
    };

    useEffect(() => {
        fetchData();
    }, []);

    return (
        <Container maxWidth="sm">
            <Dialog open={!!error} onClose={() => setError(null)}>
                <DialogTitle>Error</DialogTitle>
                <DialogContent>{error}</DialogContent>
                <DialogActions>
                    <Button onClick={() => setError(null)}>Close</Button>
                </DialogActions>
            </Dialog>
            <Typography variant="h4" gutterBottom>
                Election Interface
            </Typography>
            <Stack spacing={2}>
                <Select
                    label="Select an election to vote in"
                    value={election?.id || ""}
                    onChange={(e) => {
                        const selectedElection = elections.find(
                            (el) => el.id === e.target.value
                        );
                        setElection(selectedElection);
                    }}
                    fullWidth
                >
                    {elections.map((election) => (
                        <MenuItem key={election.id} value={election.id}>
                            {election.id}
                        </MenuItem>
                    ))}
                </Select>
                <Select
                    label="Select a candidate to vote for"
                    value={candidate}
                    onChange={(e) => setCandidate(e.target.value as string)}
                    fullWidth
                >
                    {election &&
                        election.candidates.map((cand) => (
                            <MenuItem key={cand} value={cand}>
                                {cand}
                            </MenuItem>
                        ))}
                </Select>
                <Button fullWidth onClick={submitBallot} variant="contained">
                    Vote
                </Button>
            </Stack>
            {election && (
                <div>
                    <Typography variant="h6">Current Vote Counts:</Typography>
                    <ul>
                        {election.candidates.map((cand, index) => (
                            <li key={cand}>
                                {cand}: {election.vote_counts[index]} votes
                            </li>
                        ))}
                    </ul>
                </div>
            )}
        </Container>
    );
};
