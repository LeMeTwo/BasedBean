import "./style/LogReg.css";
import "./style/Universal.css";
import Cookies from "js-cookie";
import { useNavigate } from "react-router-dom";
import { useState } from "react";
import { FaUser } from "react-icons/fa";
import { FaLock } from "react-icons/fa";

function Login() {
    // const USER_REGEX = /^[A-z][A-z0-9-_]{3,23}$/;
    // const lineToTest = "testline"
    // console.log(USER_REGEX.test(lineToTest));
    // const PASSWORD_REGEX = /^(?=.*[a-z])(?=.*[A-Z])(?=.*[0-9])(?=.*[!@#$%]).{8,24}$/;
    const [user, setUser] = useState("");
    const [password, setPwd] = useState("");
    const [result, setResult] = useState("Quickly login!");
    const navigate = useNavigate();

    const handleSubmitForm = () => {
        fetch("http://localhost:8090/login", {
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
        <div className="containerOuterForm noselect ">
            <div className="containerForm">
                <h1 className="noselect">Login</h1>

                <div className="containerInputField">
                    <FaUser className="icon" />
                    <input
                        className="inputField"
                        type="text"
                        value={user}
                        placeholder="user"
                        onChange={(e) => setUser(e.target.value)}
                    />
                </div>

                <div className="containerInputField">
                    <FaLock className="icon" />
                    <input
                        className="inputField"
                        type="password"
                        value={password}
                        placeholder="password"
                        onChange={(e) => setPwd(e.target.value)}
                    />
                </div>

                <label htmlFor="userField"></label>
                <button className="styleButton" onClick={handleSubmitForm}>
                    Submit
                </button>
                <p className="noselect">{result}</p>
            </div>
        </div>
    );
}

export default Login;
