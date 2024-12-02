import React, { useEffect, useState } from "react";
import { TextField, Button, Container, Typography, Box } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { Profile } from "../shared/models";

export const ProfilePage: React.FC = () => {
    const [name, setName] = useState("");
    const [address, setAddress] = useState("");
    const [dob, setDob] = useState("");
    const [driversLicense, setDriversLicense] = useState("");

    const handleSubmit = async (event: React.FormEvent) => {
        event.preventDefault();
        const prof: Profile = {
            first_name: name.split(" ")[0],
            last_name: name.split(" ")[1] || "",
            address,
            dob,
            driv_lic: driversLicense,
        };
        console.log(
            await invoke("create_profile", {
                profile: prof,
            })
        );
    };

    useEffect(() => {
        const fetchData = async () => {
            const prof: Profile = await invoke("get_profile");
            setName(`${prof.first_name} ${prof.last_name}`);
            setAddress(prof.address);
            setDob(prof.dob);
            setDriversLicense(prof.driv_lic);
        };
        fetchData();
    }, []);

    return (
        <Container>
            <Typography variant="h4" component="h1" gutterBottom>
                Profile
            </Typography>
            <form onSubmit={handleSubmit}>
                <Box mb={2}>
                    <TextField
                        fullWidth
                        label="Name"
                        variant="outlined"
                        value={name}
                        onChange={(e) => setName(e.target.value)}
                    />
                </Box>
                <Box mb={2}>
                    <TextField
                        fullWidth
                        label="Address"
                        variant="outlined"
                        value={address}
                        onChange={(e) => setAddress(e.target.value)}
                    />
                </Box>
                <Box mb={2}>
                    <TextField
                        fullWidth
                        label="Date of Birth"
                        type="date"
                        variant="outlined"
                        value={dob}
                        onChange={(e) => setDob(e.target.value)}
                    />
                </Box>
                <Box mb={2}>
                    <TextField
                        fullWidth
                        label="Driver's License Number"
                        variant="outlined"
                        value={driversLicense}
                        onChange={(e) => setDriversLicense(e.target.value)}
                    />
                </Box>
                <Button type="submit">Submit</Button>
            </form>
        </Container>
    );
};
