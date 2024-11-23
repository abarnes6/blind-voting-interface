import { Button } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

export const BallotPage: React.FC = () => {
    const [hasBallot, setHasBallot] = useState(false);
    const [ballotName, setBallotName] = useState("");

    useEffect(() => {
        const checkBallot = async () => {
            const exists = await invoke("ballot_exists");
            if (exists) {
                setHasBallot(true);
                const name: string = await invoke("get_ballot_name");
                setBallotName(name);
            }
        };
        checkBallot();
    }, []);
    const requestBallot = async () => {
        await invoke("request_ballot", { name: "name" });
    };
    return (
        <>
            <Button onClick={requestBallot}>Request Ballot</Button>
        </>
    );
};
