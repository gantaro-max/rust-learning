import { useState } from "react";
import type { ItemProps } from "../types";
const Item = ({name,price,stock,category,onDelete,onUpdate}:ItemProps) => {

    const [newStock,setNewStock] = useState<number>(0);

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
                在庫更新:<input type="number" value={newStock} onChange={(e)=>setNewStock(Number(e.target.value))}/>
                <p>分類:{category}</p>
            </div>
            <button onClick={()=>onUpdate(name,newStock)} style={{marginTop: "10px", cursor: "pointer", color: "white", backgroundColor: "#55e", border: "none", borderRadius: "4px", padding: "5px 10px"}}>更新</button>
            <button onClick={()=>onDelete(name)} style={{marginTop: "10px", cursor: "pointer", color: "white", backgroundColor: "#e55", border: "none", borderRadius: "4px", padding: "5px 10px"}}>削除</button>
        </>
    );
};
export default Item;