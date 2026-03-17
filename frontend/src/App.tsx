import { useEffect, useState } from 'react';
import './App.css';
import Item from './components/Item';
import type { ItemProps, UpStock } from './types';


function App() {

  const [items, setItems] = useState<ItemProps[]>([]);
  const [name, setName] = useState<string>("");
  const [price, setPrice] = useState<number>(0);
  const [stock, setStock] = useState<number>(0);
  const [category, setCategory] = useState<string>("");
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const isInvalid = name.trim() === "" || price <= 0 || stock < 0;

  useEffect(() => {

    fetch("http://localhost:8000/api/items").then(res => res.json()).then(data => setItems(data));


  }, []);

  const handleDelete = (id: number) => {
    setErrorMessage(null);

    fetch("http://localhost:8000/api/items", {
      method: "DELETE",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ id: id })
    }).then(async response => {
      if (response.ok) {
        setItems(list => list.filter(item => item.id != id));
      }
      else {
        const errorData = await response.json();
        setErrorMessage(errorData.error);
      }
    });
  }

  const handleRegister = (e: React.SubmitEvent<HTMLFormElement>) => {
    e.preventDefault();
    setErrorMessage(null);

    const newItem: ItemProps = {
      name,
      price,
      stock,
      category: category || "果物",
      onUpdate: handleUpdate,
      onDelete: handleDelete,
    }

    fetch("http://localhost:8000/api/items", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(newItem)
    }).then(async response => {
      if (response.ok) {
        const savedItem = await response.json();
        setItems(list => [...list, savedItem]);
        setName("");
        setPrice(0);
        setStock(0);
      }
      else {
        const errorData = await response.json();
        setErrorMessage(errorData.error);
        // alert("登録に失敗しました");
      }
    });

  }
  const handleUpdate = (id: number, newStock: number) => {
    setErrorMessage(null);
    const upStock: UpStock = {
      id,
      stock: newStock
    }

    fetch("http://localhost:8000/api/items", {
      method: "PATCH",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(upStock)
    }).then(async response => {
      if (response.ok) {
        setItems(list => list.map(item => item.id === id ? { ...item, stock: newStock } : item
        ));

      }
      else {
        const errorData = await response.json();
        setErrorMessage(errorData.error);
        // alert("登録に失敗しました");
      }
    });

  }




  return (
    <>
      <h1>APIから取得した商品一覧</h1>
      <div style={{ display: 'flex', flexWrap: 'wrap' }}>
        {
          items.map(item => (
            <div key={item.id}>
              <Item id={item.id} name={item.name} price={item.price} stock={item.stock} category={item.category} onDelete={(id) => handleDelete(id)} onUpdate={(id, newStock) => handleUpdate(id, newStock)} />

            </div>
          ))
        }
      </div>
      <div>
        <h3>商品の追加</h3>
        <form onSubmit={handleRegister} method="post">
          商品名：<input type="text" value={name} onChange={(e) => setName(e.target.value)} />
          <br />
          価格：<input type="number" value={price} onChange={(e) => setPrice(Number(e.target.value))} />
          <br />
          在庫数：<input type="number" value={stock} onChange={(e) => setStock(Number(e.target.value))} />
          <br />
          分類：<select name="catego" id="catego" onChange={(e) => setCategory(e.target.value)}>
            <option value="果物">果物</option>
            <option value="飲み物">飲み物</option>
            <option value="日用品">日用品</option>
          </select>
          <input type="submit" value="登録" disabled={isInvalid} style={{
            background: isInvalid ? "#ccc" : "#4caf50",
            cursor: isInvalid ? "not-allowed" : "pointer",
            color: "white",
            padding: "10px",
            border: "none",
            borderRadius: "4px"
          }} />
          <br />
          {
            isInvalid && (
              <p style={{
                color: "red",
                fontSize: "0.8rem",
                marginTop: "10px"
              }}>※商品名を入力し、価格（1円以上）と在庫（0以上）を正しく設定してください</p>
            )
          }
          {
            errorMessage && (
              <div style={{ color: "red", backgroundColor: "#ffebee", padding: "10px", marginBottom: "10px", borderRadius: "4px" }}>
                ⚠️ {errorMessage}
              </div>)
          }
          <br />
        </form>
      </div>

    </>
  )
}

export default App
