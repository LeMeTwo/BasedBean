import "./style/Paste.css";
import "./style/Universal.css";
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

function getPaste(key: string, setText: any, setTitle: any) {
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
}

function createPaste(text: string, title: string) {
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
            <div className="containerPaste noselect">
                <h1>Your Paste</h1>
                <input
                    className="styleTitle"
                    placeholder="Title"
                    type="text"
                    value={title}
                    onChange={(e) => setTitle(e.target.value)}
                />
                <textarea
                    className="stylePaste"
                    placeholder="Content..."
                    wrap="hard"
                    value={text}
                    onChange={(e) => setText(e.target.value)}
                ></textarea>
                <div className="containerButtons">
                    <button
                        className="styleButton"
                        onClick={() => createPaste(text, title)}
                    >
                        Create paste
                    </button>
                    <button
                        className="styleButton"
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
