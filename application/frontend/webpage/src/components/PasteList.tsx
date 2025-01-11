import "./style/PasteList.css";
import "./style/Universal.css";
import Cookies from "js-cookie";
import { NavigateFunction, useNavigate } from "react-router-dom";

function getPastes() {
    fetch("http://localhost:8090/user/pastes", {
        method: "GET",
        headers: { Authorization: "Bearer " + Cookies.get("token") },
    })
        .then((response) => {
            if (!response.ok) throw new Error(response.status.toString());
            return response.json();
        })
        .then((data) => {
            return data.pastes;
        })
        .catch((error: Error) => {
            console.log(Number(error.message));
        });
    return [];
}

function loadPaste(key: string, navigate: NavigateFunction) {
    navigate("/home/" + key);
}

function deletePaste(key: string) {
    fetch("http://localhost:8090/paste/" + key, {
        method: "DELETE",
        headers: { Authorization: "Bearer " + Cookies.get("token") },
        credentials: "include",
    })
        .then((response) => {
            if (!response.ok) throw new Error(response.status.toString());
        })
        .catch((error: Error) => {
            console.log(Number(error.message));
        });
    location.reload();
}

function PasteList() {
    const navigate = useNavigate();
    const list = getPastes();

    return (
        <div className="containerPasteList">
            <h1>My Pastes</h1>
            {list.length === 0 && <p>No pastes</p>}
            <ul className="containerPasteListLinks">
                {list.map((item) => (
                    <li className="noBullet" key={item.key}>
                        <button
                            className="styleButton"
                            onClick={() => loadPaste(item.key, navigate)}
                        >
                            {item.title}
                        </button>
                        <button
                            className="styleButton"
                            onClick={() => deletePaste(item.key)}
                        >
                            Delete
                        </button>
                    </li>
                ))}
            </ul>
        </div>
    );
}

export default PasteList;
