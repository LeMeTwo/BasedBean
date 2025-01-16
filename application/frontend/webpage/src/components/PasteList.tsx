import "./style/PasteList.css";
import "./style/Universal.css";
import Cookies from "js-cookie";
import { NavigateFunction, useNavigate } from "react-router-dom";

const ip = import.meta.env.VITE_SERVER_IP;
const port = import.meta.env.VITE_SERVER_PORT;

function getPastes() {
    fetch("http://" + ip + ":" + port + "/user/pastes", {
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
    fetch("http://" + ip + ":" + port + "/paste/" + key, {
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
        <div className="container-pastelist">
            {list.length === 0 && <p>No pastes</p>}
            <ul className="container-pastelist__links">
                {list.map((item: any) => (
                    <li className="noBullet" key={item.key}>
                        <button
                            className="button"
                            onClick={() => loadPaste(item.key, navigate)}
                        >
                            {item.title}
                        </button>
                        <button
                            className="button"
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
