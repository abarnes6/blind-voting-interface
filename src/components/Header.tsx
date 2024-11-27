import React from "react";
import { Typography, AppBar, Toolbar, Button } from "@mui/material";

const Header: React.FC = () => {
    return (
        <AppBar position="static" style={{ marginBottom: "20px" }}>
            <Toolbar>
                <Typography variant="h6" style={{ flexGrow: 1 }}>
                    Secure Election System
                </Typography>
                <Button color="inherit" component="a" href="/profile">
                    Profile
                </Button>
                <Button color="inherit" component="a" href="/">
                    Vote
                </Button>
                <Button color="inherit" component="a" href="/ballot">
                    Ballot
                </Button>
            </Toolbar>
        </AppBar>
    );
};

export default Header;
