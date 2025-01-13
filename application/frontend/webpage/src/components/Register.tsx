import "./style/LogReg.css";
import "./style/Universal.css";
import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { FaUser } from "react-icons/fa";
import { FaLock } from "react-icons/fa";

function Register() {
    const USER_REGEX = /^[A-z0-9-_]$/;

    // console.log(USER_REGEX.test(lineToTest));
    const PASSWORD_REGEX_CAP = /[A-Z]+/;
    const PASSWORD_REGEX_LOW = /[a-z]+/;
    const PASSWORD_REGEX_NUM = /[0-9]+/;

    const [user, setUser] = useState("");
    const [password, setPwd] = useState("");
    const [result, setResult] = useState("Quickly register!");
    const [userObs, setUserObs] = useState("");
    const [pwdObs, setPwdObs] = useState("");

    const [flagUser, setFlagUser] = useState(false);
    const [flagPwd, setFlagPwd] = useState(false);

    const navigate = useNavigate();

    function userObserver(usr: string) {
        setUser(usr);
        let flagUsrChar = false;
        let flagUsrLen = false;

        if (USER_REGEX.test(usr)) {
            console.log("Allowed A-z,0-9,-_");
            flagUsrChar = true;
        } else {
            console.log("Wrong");
        }

        if (usr.length == 0) {
            setUserObs("");
        } else if (usr.length < 4) {
            setUserObs("Username too short");
        } else if (usr.length > 20) {
            setUserObs("Username too long");
        } else {
            flagUsrLen = true;
            setUserObs(usr.length.toString());
        }

        setFlagUser(flagUsrChar && flagUsrLen);
    }

    function pwdObserver(pwd: string) {
        setPwd(pwd);
        let flagPwdLen = false;
        let flagPwdNum = false;
        let flagPwdCap = false;
        let flagPwdLow = false;

        if (PASSWORD_REGEX_CAP.test(pwd)) {
            console.log("At least one upper case letter");
            flagPwdCap = true;
        } else {
            console.log("Wrong");
        }

        if (PASSWORD_REGEX_LOW.test(pwd)) {
            console.log("At least one smol lowercase letter");
            flagPwdLow = true;
        } else {
            console.log("Wrong");
        }
        if (PASSWORD_REGEX_NUM.test(pwd)) {
            console.log("At least one number");
            flagPwdNum = true;
        } else {
            console.log("Wrong");
        }

        if (pwd.length == 0) {
            setPwdObs("");
        } else if (pwd.length < 4) {
            setPwdObs("Password too short");
        } else {
            setPwdObs(pwd.length.toString());
            flagPwdLen = true;
        }
        setFlagPwd(flagPwdCap && flagPwdLow && flagPwdNum && flagPwdLen);
    }

    const handleSubmitForm = () => {
        if (!flagUser || !flagPwd) {
            setResult("Invalid data");
            return;
        }

        fetch("http://localhost:8090/register", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ user, password }),
            credentials: "include",
        })
            .then((response) => {
                if (!response.ok) throw new Error(response.status.toString());
                else {
                    feedBack(response.status);
                    navigate("/login");
                }
                return response.json();
            })
            .catch((error) => {
                feedBack(error.status);
                console.log(error);
            });

        setUser("");
        setPwd("");

        setUserObs("");
        setPwdObs("");
    };

    const feedBack = (check: number) => {
        if (check == 201) {
            setResult("Creation succesfull");
        } else {
            setResult("User name already taken");
        }
    };

    return (
        <div className="containerOuterForm">
            <div className="containerForm">
                <h1 className="noselect">Register</h1>
                <div className="containerInputField">
                    <FaUser className="icon" />
                    <input
                        className="inputField"
                        type="text"
                        value={user}
                        placeholder="user"
                        onChange={(e) => userObserver(e.target.value)}
                    />
                </div>
                <label>{userObs}</label>

                <div className="containerInputField">
                    <FaLock className="icon" />
                    <input
                        className="inputField"
                        type="password"
                        value={password}
                        placeholder="password"
                        onChange={(e) => pwdObserver(e.target.value)}
                    />
                </div>
                <label>{pwdObs}</label>

                <label htmlFor="userField"></label>
                <button className="styleButton" onClick={handleSubmitForm}>
                    Submit
                </button>
                <p className="noselect">{result}</p>
            </div>
        </div>
    );
}

export default Register;
