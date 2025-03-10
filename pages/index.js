import React,{useState,useEffect} from "react";
import {MdDelete, MdEdit, MdConfirmationNumber} from 'react-icons/md'
import axios from 'axios'
import {format} from 'date-fns'


const index = () => {
  const [editText, setEditText] = useState();
  const [todos, setTodos] = useState([]);
  const [todosCopy, setTodosCopy] = useState(todos);
  const [todoInput, setTodoInput] = useState("");
  const [editIndex, setEditIndex] = useState(-1);
  const [searchInput, setSearchInput] = useState("");
  const [searchResults, setSearchResults] = useState([]);
  
  const [count, setCount] = useState(0);
  const [search, setSearch] = useState("");
  const [searchItem, setSearchItem] = useState(search);
  useEffect(() => {
  }, [count]);

  const editTodo = (index) => {
    setTodoInput(todos[index].title);
    setEditIndex(index);
  }

  const fetchTodos = async () => {
    try{
      const response = await axios.get('http://127.0.0.1:8000/todos')
      setTodos(response.data);
      setTodosCopy(response.data);
    }catch(error){
      console.log(error)
    }
  }

  const addTodo = async () => {
    try{
      if(editIndex===-1){
        const response = await axios.get('http://127.0.0.1:8000/todos',{
          title:todoInput.title,
          completed:false
        })
        setTodos(response.data);
        setTodosCopy(response.data);
        setTodoInput("");
      }else{
        const todoToUpdate = {... todos[editIndex],title:todoInput};
        ;
        const response = await axios.put(`http://127.0.0.1:8000/todos/${todoToUpdate.id}`,{
          todoToUpdate
        });

        console.log(response.data);
        const updatedTodos=[...todos];
        updatedTodos[editIndex]=response.data;
        setTodos(updatedTodos);
        // setTodosCopy(updatedTodos);
        setTodoInput("");
        setEditIndex(-1);
        setCount(count+1);
      }
    
    }catch(error){
      console.log(error)
    }
  }

  const deleteTodo = async (id) => {
    try{
      // const todoToUpdate = {... todos[editIndex],title:todoInput};
        
        const response = await axios.delete(`http://127.0.0.1:8000/todos/${id}`,{
          
        });
        setTodos(todos.filter((todo)=>todo.id!==id));

    }catch(error){
      console.log(error)
    }
  }
    
  const toggleCompleted = async (index) => {
    try{
      const todoToUpdate = {... todos[editIndex],completed:!todos[index].completed};
        
        const response = await axios.delete(`http://127.0.0.1:8000/todos/${todoToUpdate.id}`,{
          
        });
        const updatedTodos=[...todos];
        updatedTodos[index]=response.data;
        setTodos(updatedTodos);
        setCount(count+1);
    }catch(error){
      console.log(error)
    }
  }

  const searchTodo = () => {
    const results = todos.filter((todo) => {
      return todo.title.toLowerCase().includes(searchInput.toLowerCase());
    });
    setSearchResults(results);
  }

  

  return <div>index1</div>;
};

export default index;
