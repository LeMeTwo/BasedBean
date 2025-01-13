import { Outlet, Link, useNavigate } from "react-router-dom";
import Cookies from "js-cookie";
import "./style/Navbar.css";
import "./style/Universal.css";
function Navbar() {
    const navigate = useNavigate();

    function IsTokenThere() {
        if (Cookies.get("token") == undefined) {
            return (
                <>
                    <Link to="login">
                        <button className="styleButton noselect">Login</button>
                    </Link>

                    <Link to="register">
                        <button className="styleButton noselect">
                            Register
                        </button>
                    </Link>
                </>
            );
        } else {
            return (
                <>
                    <Link to="profile">
                        <button className="styleButton noselect">
                            Profile
                        </button>
                    </Link>
                    <button
                        className="styleButton"
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
            <div className="containerNav">
                <Link to="/home">
                    <button className="styleButton">Home</button>
                </Link>

                <IsTokenThere />
            </div>
            <Outlet />
        </>
    );
}

export default Navbar;
