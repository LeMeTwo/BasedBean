import "./style/LogReg.css";
import "./style/Universal.css";
import { useState } from "react";
import { useNavigate } from "react-router";
import { FaUser } from "react-icons/fa";
import { FaLock } from "react-icons/fa";

function Register() {
    // const USER_REGEX = /^[A-z][A-z0-9-_]{3,23}$/;
    const USER_REGEX_FIRST_CHAR = /^[A-z]/;

    // const lineToTest = "testline"
    // console.log(USER_REGEX.test(lineToTest));
    // const PASSWORD_REGEX = /^(?=.*[a-z])(?=.*[A-Z])(?=.*[0-9])(?=.*[!@#$%]).{8,24}$/;
    const [user, setUser] = useState("");
    const [password, setPwd] = useState("");
    const [result, setResult] = useState("Quickly register!");
    const [userObs, setUserObs] = useState("");
    const [pwdObs, setPwdObs] = useState("");

    const navigate = useNavigate();

    function userObserver(usr: string) {
        setUser(usr);
        // console.log(usr.length)

        if (USER_REGEX_FIRST_CHAR.test(usr)) {
            console.log("Starts with letter");
        } else {
            console.log("Wrong");
        }

        if (usr.length == 0) {
            setUserObs("");
        } else if (usr.length < 4) {
            setUserObs("Username too short");
        } else {
            setUserObs(usr.length.toString());
        }
    }

    function pwdObserver(pwd: string) {
        setPwd(pwd);
        // console.log(usr.length)
        if (pwd.length == 0) {
            setPwdObs("");
        } else if (pwd.length < 4) {
            setPwdObs("Password too short");
        } else {
            setPwdObs(pwd.length.toString());
        }
    }

    const handleSubmitForm = () => {
        // console.log(JSON.stringify({user,password}))
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
