import "./style/Navbar.css";
import "./style/Universal.css";
import { Link, useNavigate } from "react-router-dom";
import Cookies from "js-cookie";

function Navbar() {
    const navigate = useNavigate();

    function IsTokenThere() {
        if (Cookies.get("token") == undefined) {
            return (
                <>
                    <Link to="login">
                        <button className="button noselect">Login</button>
                    </Link>

                    <Link to="register">
                        <button className="button noselect">Register</button>
                    </Link>
                </>
            );
        } else {
            return (
                <>
                    <Link to="profile">
                        <button className="button noselect">Profile</button>
                    </Link>
                    <button
                        className="button"
                        onClick={() => {
                            Cookies.remove("token");
                            navigate("/");
                            location.reload();
                        }}
                    >
                        Logout
                    </button>
                </>
            );
        }
    }

    return (
        <>
            <div className="container-nav noselect">
                <Link to="/home">
                    <button className="button">Home</button>
                </Link>
                <IsTokenThere />
            </div>
        </>
    );
}

export default Navbar;
