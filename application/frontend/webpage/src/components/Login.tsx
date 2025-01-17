import "./style/LogReg.css";
import "./style/Universal.css";
import Cookies from "js-cookie";
import { useNavigate } from "react-router-dom";
import { useState } from "react";
import { FaUser } from "react-icons/fa";
import { FaLock } from "react-icons/fa";

const ip = import.meta.env.VITE_SERVER_IP;
const port = import.meta.env.VITE_SERVER_PORT;

function Login() {
    const [user, setUser] = useState("");
    const [password, setPwd] = useState("");
    const [result, setResult] = useState("");
    const navigate = useNavigate();

    const handleSubmitForm = () => {
        fetch("http://" + ip + ":" + port + "/login", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ user, password }),
            credentials: "include",
        })
            .then((response: Response) => {
                if (!response.ok) throw new Error(response.status.toString());
                else {
                    feedBack(response.status);
                }
                return response.json();
            })
            .then((data) => {
                console.log("data catched");
                Cookies.set("token", data.token, {
                    path: "/",
                    sameSite: "strict",
                    expires: 1000,
                });
                navigate("/");
                location.reload();
            })
            .catch((error: Error) => {
                feedBack(Number(error.message));
            });

        setUser("");
        setPwd("");
    };

    const feedBack = (check: number) => {
        if (check == 200) {
            setResult("Login succesfull");
        } else if (check == 401) {
            setResult("Invalid login or password");
        } else {
            console.log(check);
            setResult("Server error");
        }
    };

    return (
        <>
            <div className="container-credentials">
                <h1 className="noselect">Login</h1>

                <div className="container-credentials__inputs">
                    <FaUser className="icon" />
                    <input
                        className="input-field"
                        type="text"
                        value={user}
                        placeholder="user"
                        onChange={(e) => setUser(e.target.value)}
                    />
                </div>

                <div className="container-credentials__inputs">
                    <FaLock className="icon" />
                    <input
                        className="input-field"
                        type="password"
                        value={password}
                        placeholder="password"
                        onChange={(e) => setPwd(e.target.value)}
                    />
                </div>

                <label htmlFor="userField"></label>
                <button className="button" onClick={handleSubmitForm}>
                    Submit
                </button>
                <p className="noselect">{result}</p>
            </div>
        </>
    );
}

export default Login;
