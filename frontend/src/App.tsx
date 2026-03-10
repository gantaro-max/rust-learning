import { useEffect, useState } from 'react';
import './App.css';
import Item from './components/Item';
import type { ItemProps, UpStock } from './types';


function App() {

  const [items,setItems] = useState<ItemProps[]>([]);
  const [name,setName] = useState<string>("");
  const [price,setPrice] = useState<number>(0);
  const [stock,setStock] = useState<number>(0);
  const [category,setCategory] = useState<string>("");

  useEffect(()=>{

    fetch("http://localhost:8000/api/items").then(res => res.json()).then(data => setItems(data));


  },[]);

  const handleDelete = (name:string) => {

    fetch("http://localhost:8000/api/items",{
      method:"DELETE",
      headers:{"Content-Type":"application/json"},
      body:JSON.stringify(name)
    }).then(response=>{
      if(response.ok){
        setItems(list => list.filter(item => item.name!=name));
      }
    });
  }

  const handleRegister = (e: React.SubmitEvent<HTMLFormElement> )=>{
    e.preventDefault();

    const newItem:ItemProps = {
      name,
      price,
      stock,
      category: category || "果物",
      onDelete:handleDelete,
      onUpdate:handleUpdate
    }

    fetch("http://localhost:8000/api/items",{
      method:"POST",
      headers:{"Content-Type":"application/json"},
      body:JSON.stringify(newItem)
    }).then(response=>{
      if(response.ok){
        setItems(list=> [...list,newItem]);
        setName("");
        setPrice(0);
        setStock(0);
      }
      else{
        alert("登録に失敗しました");
      }
    });

  }
  const handleUpdate =(name:string,newStock:number) =>{
    const upStock:UpStock = {
      name,
      stock
    }

    fetch("http://localhost:8000/api/items",{
      method:"PATCH",
      headers:{"Content-Type":"application/json"},
      body:JSON.stringify(upStock)
    }).then(response=>{
      if(response.ok){
        setItems(list=> list.map(item=> item.name===name ? {...item,stock:newStock}:item
        ));
        
      }
      else{
        alert("登録に失敗しました");
      }
    });
    
  }

  
  

  return (
    <>
      <h1>APIから取得した商品一覧</h1>
      <div style={{display:'flex'}}>
        {
          items.map(item => (
            <div key={item.name}>
                <Item name={item.name} price={item.price} stock={item.stock} category={item.category} onDelete={(name) => handleDelete(name)} onUpdate={(name,newStock)=>handleUpdate(name,newStock)}/>
              
            </div>
        ))
        }
      </div>
      <div>
        <h3>商品の追加</h3>
        <form onSubmit={handleRegister} method="post">
          商品名：<input type="text" value={name} onChange={(e)=>setName(e.target.value)} />
          <br />
          価格：<input type="number" value={price} onChange={(e)=>setPrice(Number(e.target.value))} />
          <br />
          在庫数：<input type="number" value={stock} onChange={(e)=>setStock(Number(e.target.value))} />
          <br />
          分類：<select name="catego" id="catego" onChange={(e)=>setCategory(e.target.value)}>
            <option value="果物">果物</option>
            <option value="飲み物">飲み物</option>
            <option value="日用品">日用品</option>
          </select>

          <input type="submit" value="登録" />

        </form>
      </div>
          
    </>
  )
}

export default App
