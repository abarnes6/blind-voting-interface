import React, { useState } from "react";
import { TextField, Button, Container, Typography, Box } from "@mui/material";

export const ProfilePage: React.FC = () => {
    const [name, setName] = useState("");
    const [address, setAddress] = useState("");
    const [dob, setDob] = useState("");
    const [driversLicense, setDriversLicense] = useState("");

    const handleSubmit = (event: React.FormEvent) => {
        event.preventDefault();
        // Handle form submission logic here
        console.log({ name, address, dob, driversLicense });
    };

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
