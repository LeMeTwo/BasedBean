import "./style/Profile.css";
import PasteList from "./PasteList";

function Profile() {
    return (
        <div className="container-profile">
            <h1>My Pastes</h1>
            <PasteList />
        </div>
    );
}

export default Profile;
