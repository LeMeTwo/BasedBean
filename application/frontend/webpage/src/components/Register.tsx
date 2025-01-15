import "./style/LogReg.css";
import "./style/Universal.css";
import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { FaUser } from "react-icons/fa";
import { FaLock } from "react-icons/fa";

function passwordChecker(pwd: string, setPassword: any, setPassFlag: any) {
    setPassword(pwd);

    const PASSWORD_REGEX_CAP = /[A-Z]+/;
    const PASSWORD_REGEX_LOW = /[a-z]+/;
    const PASSWORD_REGEX_NUM = /[0-9]+/;

    let flagPwdLen = false;
    let flagPwdNum = false;
    let flagPwdCap = false;
    let flagPwdLow = false;

    // Check for at least one capital letter
    if (PASSWORD_REGEX_CAP.test(pwd)) {
        flagPwdCap = true;
        document.getElementById("pwdUpper")!.style.color = "#FFF0D1";
    } else {
        document.getElementById("pwdUpper")!.style.color = "#d55e5e";
    }
    // Check for at least one lowercase letter
    if (PASSWORD_REGEX_LOW.test(pwd)) {
        flagPwdLow = true;
        document.getElementById("pwdLower")!.style.color = "#FFF0D1";
    } else {
        document.getElementById("pwdLower")!.style.color = "#d55e5e";
    }
    // Check for at least one number
    if (PASSWORD_REGEX_NUM.test(pwd)) {
        flagPwdNum = true;
        document.getElementById("pwdNum")!.style.color = "#FFF0D1";
    } else {
        document.getElementById("pwdNum")!.style.color = "#d55e5e";
    }
    // Check for length > 6
    if (pwd.length > 6) {
        flagPwdLen = true;
        document.getElementById("pwdLen")!.style.color = "#FFF0D1";
    } else {
        document.getElementById("pwdLen")!.style.color = "#d55e5e";
    }
    setPassFlag(flagPwdCap && flagPwdLow && flagPwdNum && flagPwdLen);
}

function userChecker(usr: string, setUsername: any, setUserFlag: any) {
    setUsername(usr);

    const USER_REGEX = /^[A-Za-z0-9-_]+/;

    let flagUsrChar = false;
    let flagUsrLen = false;

    // Check for invalid characters
    if (USER_REGEX.test(usr)) {
        document.getElementById("usrValid")!.style.color = "#FFF0D1";
        flagUsrChar = true;
    } else {
        document.getElementById("usrValid")!.style.color = "#d55e5e";
    }

    // Check username length
    if (usr.length > 3 && usr.length < 20) {
        flagUsrLen = true;
        document.getElementById("usrLen")!.style.color = "#FFF0D1";
    } else {
        document.getElementById("usrLen")!.style.color = "#d55e5e";
    }

    setUserFlag(flagUsrChar && flagUsrLen);
}

const feedBack = (check: number, setResult: any) => {
    if (check == 201) {
        setResult("Creation succesfull");
    } else {
        setResult("User name already taken");
    }
};

function Register() {
    const [user, setUser] = useState("");
    const [password, setPwd] = useState("");

    const [result, setResult] = useState("");

    const [flagUser, setFlagUser] = useState(false);
    const [flagPwd, setFlagPwd] = useState(false);

    const navigate = useNavigate();

    function submitCredentials() {
        if (!flagUser || !flagPwd) {
            setResult("Invalid Credentials");
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
                    feedBack(response.status, setResult);
                    navigate("/login");
                }
                return response.json();
            })
            .catch((error) => {
                feedBack(error.status, setResult);
                console.log(error);
            });

        // Reset user and password fields
        setUser("");
        setPwd("");
    }

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
                        onChange={(e) =>
                            userChecker(e.target.value, setUser, setFlagUser)
                        }
                    />
                </div>
                <ul className="noselect listRules">
                    <li id="usrValid">Valid characters</li>
                    <li id="usrLen">
                        Longer than 3 characters
                        <br /> shorter than 20
                    </li>
                </ul>

                <div className="containerInputField">
                    <FaLock className="icon" />
                    <input
                        className="inputField"
                        type="password"
                        value={password}
                        placeholder="password"
                        onChange={(e) =>
                            passwordChecker(e.target.value, setPwd, setFlagPwd)
                        }
                    />
                </div>

                <ul className="noselect listRules">
                    <li id="pwdUpper">One upper case letter</li>
                    <li id="pwdLower">One lower case letter</li>
                    <li id="pwdNum">One number</li>
                    <li id="pwdLen">Longer than 6 characters</li>
                </ul>

                <label htmlFor="userField"></label>
                <button className="styleButton" onClick={submitCredentials}>
                    Submit
                </button>
                <p className="noselect">{result}</p>
            </div>
        </div>
    );
}

export default Register;
