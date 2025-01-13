import "./style/Home.css";
import Paste from "./Paste";
import { useParams } from "react-router-dom";
function Home() {
    const { key } = useParams();
    return (
        <>
            <div className="containerOuterHome">
                <Paste pasteKey={key} />
            </div>
        </>
    );
}

export default Home;
