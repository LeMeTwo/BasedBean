import "./style/Paste.css";
import { useState } from "react";
import { useParams } from "react-router-dom";

function Paste() {
    const [title, setTitle] = useState("");
    const [text, setText] = useState("");

    const handleAuthorizeUser = () => {
        // console.log(JSON.stringify({user,password}))
        fetch("http://localhost:8090/paste", {
            method: "POST",
            headers: { "Content-Type": "authorization" },
            body: JSON.stringify({ text, title }),
            credentials: "include",
        })
            .then((response) => {
                if (!response.ok) throw new Error(response.status.toString());
                else {
                    feedBack(response.status);
                }
                return response.json();
            })
            .catch((error) => {
                feedBack(error.status);
                console.log(error);
            });
    };

    const handleSavePaste = () => {
        // console.log(JSON.stringify({user,password}))
        fetch("http://localhost:8090/paste", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ text, title }),
            credentials: "include",
        })
            .then((response) => {
                if (!response.ok) throw new Error(response.status.toString());
                else {
                    feedBack(response.status);
                }
                return response.json();
            })
            .catch((error) => {
                feedBack(error.status);
                console.log(error);
            });
    };

    const savePaste = () => {
        console.log("saved");
    };

    const cleanPaste = () => {
        setText("");
    };

    const feedBack = (check: number) => {
        if (check == 201) {
            console.log("Paste save succesfull");
        } else {
            console.log("Couldn't save paste");
        }
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
