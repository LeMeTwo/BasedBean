import "./style/PasteList.css";
import "./style/Universal.css";
import Cookies from "js-cookie";
import { NavigateFunction, useNavigate } from "react-router-dom";
import { useState } from "react";

const ip = "localhost";
const port = "8090";

function getPastes(setList: any, setLoading: any) {
    fetch("http://" + ip + ":" + port + "/user/pastes", {
        method: "GET",
        headers: { Authorization: "Bearer " + Cookies.get("token") },
    })
        .then((response) => {
            if (!response.ok) throw new Error(response.status.toString());
            return response.json();
        })
        .then((data) => {
            setLoading(false);
            setList(data.pastes);
            return data;
        })
        .catch((error: Error) => {
            console.log(Number(error.message));
        });
}

function loadPaste(key: string, navigate: NavigateFunction) {
    navigate("/home/" + key);
}

function deletePaste(key: string) {
    fetch("http://" + ip + ":" + "8090" + "/paste/" + key, {
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
    console.log("Paste Loaded");
    const [isLoading, setLoading] = useState(true);
    const [list, setList] = useState([]); // Loading state
    if (isLoading) {
        getPastes(setList, setLoading);
    }
    const navigate = useNavigate();

    return (
        <div className="container-pastelist">
            <ul className="container-pastelist__links">
                {list.length === 0 && <h1>No pastes to load</h1>}
                {list.map((item: any) => (
                    <li
                        className="container-pastelist__le noBullet"
                        key={item.key}
                    >
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
