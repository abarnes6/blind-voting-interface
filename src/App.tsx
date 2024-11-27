import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import Header from "./components/Header";
import { VotePage } from "./pages/VotePage.tsx";
import { ProfilePage } from "./pages/ProfilePage.tsx";
import { BallotPage } from "./pages/BallotPage.tsx";

function App() {
    return (
        <Router>
            <Header />
            <Routes>
                <Route path="/" element={<VotePage />} />
                <Route path="/profile" element={<ProfilePage />} />
                <Route path="/ballot" element={<BallotPage />} />
            </Routes>
        </Router>
    );
}

export default App;
