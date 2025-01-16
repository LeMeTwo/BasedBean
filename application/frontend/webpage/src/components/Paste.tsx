import "./style/Paste.css";
import "./style/Universal.css";
import { useState } from "react";
import Cookies from "js-cookie";

const ip = import.meta.env.VITE_SERVER_IP;
const port = import.meta.env.VITE_SERVER_PORT;

function getHeaders() {
    const headers = new Headers();
    headers.append("Content-Type", "application/json");

    const token: string | undefined = Cookies.get("token");
    if (token !== undefined) {
        headers.append("Authorization", "Bearer " + token);
    }

    return headers;
}

function getPaste(key: string, setText: any, setTitle: any) {
    fetch("http://" + ip + ":" + port + "/paste/" + key, {
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
}

function createPaste(text: string, title: string) {
    fetch("http://" + ip + ":" + port + "/paste", {
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
}

function cleanContent(setText: any) {
    setText("");
}

function Paste({ pasteKey }: any) {
    const [title, setTitle] = useState("");
    const [text, setText] = useState("");

    if (pasteKey !== undefined) {
        getPaste(pasteKey, setText, setTitle);
    }

    return (
        <>
            <div className="container-paste noselect">
                <h1 className="container-paste__header">Your Paste</h1>
                <input
                    className="input-title"
                    placeholder="Title"
                    type="text"
                    value={title}
                    onChange={(e) => setTitle(e.target.value)}
                />

                <textarea
                    className="input-text"
                    placeholder="Content..."
                    wrap="hard"
                    value={text}
                    onChange={(e) => setText(e.target.value)}
                ></textarea>

                <div className="container-paste__buttons">
                    <button
                        className="button"
                        onClick={() => createPaste(text, title)}
                    >
                        Create paste
                    </button>

                    <button
                        className="button"
                        onClick={() => cleanContent(setText)}
                    >
                        Clean content
                    </button>
                </div>
            </div>
        </>
    );
}

export default Paste;
