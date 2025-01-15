import "./style/Universal.css";
import Paste from "./Paste";
import { useParams } from "react-router-dom";
function Home() {
    const { key } = useParams();
    return (
        <>
            <Paste pasteKey={key} />
        </>
    );
}

export default Home;
