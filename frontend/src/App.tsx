import { useEffect, useState } from 'react';
import './App.css';
import Item from './components/Item';
import type { ItemProps } from './types';

function App() {

  const [items,setItems] = useState<ItemProps[]>([]);


  useEffect(()=>{

    fetch("http://localhost:8000/api/items").then(res => res.json()).then(data => setItems(data));


  },[]);
  

  return (
    <>
      <h1>APIから取得した商品一覧</h1>
      <div style={{display:'flex'}}>
        {
          items.map(item => (
            <div key={item.name}>
                <Item name={item.name} price={item.price} stock={item.stock} category={item.category}/>
              
            </div>
        ))
        }
      </div>
          
    </>
  )
}

export default App
