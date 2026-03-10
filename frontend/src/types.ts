export type ItemProps = {
    name:string,
    price:number,
    stock:number,
    category:string,
    onDelete:(name:string)=>void,
    onUpdate:(name:string,newStock:number)=>void
};

export type UpStock ={
    name:string,
    stock:number,
};

