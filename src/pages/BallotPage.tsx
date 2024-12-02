import { Button } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { Ballot, Profile } from "../shared/models";

export const BallotPage: React.FC = () => {
    const [ballot, setBallot] = useState<Ballot>();

    const [profile, setProfile] = useState<Profile | undefined>();

    useEffect(() => {
        const loadData = async () => {
            setProfile(await invoke("get_profile"));
            const ballot: Ballot = await invoke("get_ballot");
            if (ballot.id === "None") {
                return;
            }
            setBallot(ballot);
        };
        loadData();
    }, []);

    const requestBallot = async () => {
        if (!profile) {
            console.error("Profile not set");
            return;
        }
        const ballot: Ballot = await invoke("request_ballot", {
            id: profile.first_name + " " + profile.last_name,
        });
        if (ballot.id === "None") {
            return;
        }
        setBallot(ballot);
    };

    return (
        <>
            {ballot ? (
                <div>
                    <h2>Your Ballot</h2>
                    <p>Ballot ID: {ballot.id}</p>
                    <p>Ballot Details: {ballot.signature}</p>
                </div>
            ) : (
                <div style={{ display: "flex", justifyContent: "center" }}>
                    <Button onClick={requestBallot}>Request Ballot</Button>
                </div>
            )}
        </>
    );
};
