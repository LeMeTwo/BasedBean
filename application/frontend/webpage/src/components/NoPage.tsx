import dingus from "../assets/error.png";

function NoPage() {
    return (
        <>
            <h1>404 Wrong Page !!!</h1>
            <img src={dingus} style={{ width: "404px" }} />
        </>
    );
}
export default NoPage;
