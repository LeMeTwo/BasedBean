import "./style/Layout.css";
import Navbar from "./Navbar";
import { Outlet } from "react-router-dom";

function Layout() {
    return (
        <>
            <Navbar></Navbar>
            <div className="container-layout">
                <Outlet />
            </div>
        </>
    );
}
export default Layout;
