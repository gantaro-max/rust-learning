export type ItemProps = {
    id?:number,
    name:string,
    price:number,
    stock:number,
    category:string,
    onDelete:(id:number)=>void,
    onUpdate:(id:number,newStock:number)=>void
};

export type UpStock ={
    id:number,
    stock:number,
};

