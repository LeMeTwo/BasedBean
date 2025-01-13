import "./style/Paste.css";
import { useState } from "react";
import Cookies from "js-cookie";

function getHeaders() {
    const headers = new Headers();
    headers.append("Content-Type", "application/json");

    const token: string | undefined = Cookies.get("token");
    if (token !== undefined) {
        headers.append("Authorization", "Bearer " + token);
    }

    return headers;
}

function Paste({ pasteKey }) {
    const [title, setTitle] = useState("");
    const [text, setText] = useState("");

    const getPaste = (key: string, setText: any, setTitle: any) => {
        fetch("http://localhost:8090/paste/" + key, {
            method: "GET",
        })
            .then((response) => {
                if (!response.ok) throw new Error(response.status.toString());
                return response.json();
            })
            .then((data) => {
                setText(data.text);
                setTitle(data.title);
            })
            .catch((error: Error) => {
                console.log(Number(error.message));
            });
    };

    if (pasteKey !== undefined) {
        getPaste(pasteKey, setText, setTitle);
    }

    const savePaste = () => {
        fetch("http://localhost:8090/paste", {
            method: "POST",
            headers: getHeaders(),
            body: JSON.stringify({ text, title }),
            credentials: "include",
        })
            .then((response) => {
                if (!response.ok) throw new Error(response.status.toString());
                return response.json();
            })
            .catch((error: Error) => {
                console.log(Number(error.message));
            });
    };

    const cleanPaste = () => {
        setText("");
    };

    return (
        <>
            <div className="containerPaste">
                <h1 className="noselect">Your Paste</h1>
                <input
                    className="styleTitle noselect"
                    placeholder="Title"
                    type="text"
                    value={title}
                    onChange={(e) => setTitle(e.target.value)}
                />
                <textarea
                    className="stylePaste noselect"
                    placeholder="Content..."
                    wrap="hard"
                    value={text}
                    onChange={(e) => setText(e.target.value)}
                ></textarea>
                <div className="containerButtons">
                    <button
                        className="styleButton noselect"
                        onClick={savePaste}
                    >
                        Create paste
                    </button>
                    <button
                        className="styleButton noselect"
                        onClick={cleanPaste}
                    >
                        Clean paste
                    </button>
                </div>
            </div>
        </>
    );
}

export default Paste;
