import type { ItemProps } from "../types";
const Item = ({name,price,stock,category}:ItemProps) => {

    const cardStyle = {
        border: "1px solid #ddd",
        borderRadius: "8px",
        padding: "16px",
        margin: "10px",
        backgroundColor: "#f9f9f9",
        width: "200px"
    };

    return(
        <>
            <div style={cardStyle}>
                <h3>{name}</h3>
                <p style={{color:"#929"}}>価格:{price}</p>
                
                <p style={{color:stock!=0 ?"#f25":"red", fontWeight:stock!=0?"normal":"bold"}}>在庫：{stock!=0?stock:"売り切れ"}</p>
                <p>分類:{category}</p>
            </div>
        </>
    );
};
export default Item;