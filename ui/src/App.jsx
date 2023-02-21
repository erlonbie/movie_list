import { Button, Container, Flex, Input, Item, Spacer } from './styles'
import { useEffect, useState } from 'react'
import axios from 'axios'

function App() {
  const [movieText, setMovieText] = useState('')
  const [movieItem, setMovieItem] = useState([])

  useEffect(() => {
    axios
      .get('http://localhost:8000/blog-posts')
      .then((response) => {
        setMovieItem(response.data)
      })
      .catch((error) => {
        console.error('Failed to fetch data', error)
      })
  }, [])

  const addMovie = (db) => {
    if (!movieText) return alert('Preencha um filme')
    const title = movieText.trim().toUpperCase();
    const newMovie = {
      title: title,
      body: 'Sample body',
      published: db == 'db1' ? true : false,
    }
    axios.post(`http://localhost:8000/blog-posts/${db}`, JSON.stringify(newMovie)).then(
      (response) => {
        console.log('Success:', response.data)
        setMovieItem([...movieItem, newMovie])
        setMovieText('')
      }
    ).catch(error => {
        console.error('Error:', error);
        return alert('Não é permitido itens duplicados')
        // Do something with the error, like show an error message
      });
  }

  const removeTask = (id) => {
    const newList = movieItem.filter((movieText) => movieText.id !== id)
    setMovieItem(newList)
  }

  const toggleChecked = (id, checked) => {
    const index = movieItem.findIndex((movieText) => movieText.id === id)
    const newList = movieItem
    newList[index].checked = !checked
    setMovieItem([...newList])
  }

  return (
    <Container>
      <h1 className='title'>TODO LIST</h1>
      <Spacer />

      <Flex direction='row'>
        <Input
          value={movieText}
          placeholder='Digite um filme'
          onChange={(e) => setMovieText(e.target.value)}
        />
        <Button id='bd1' onClick={() => addMovie('db1')}>BD1</Button>
        <Button id='bd2' onClick={() => addMovie('db2')}>BD1</Button>
      </Flex>
      <Spacer margin='16px' />

      <ul>
        {movieItem.map((movieText) => (
          <>
            <Item Item key={movieText.id}>
              <p>{movieText.title}</p>
              <Flex direction='row'>
                <button onClick={() => removeTask('db1', movieText.id) } >
                  <i className='bx bx-trash-alt '></i>
                </button>
                <button onClick={() => removeTask('db2', movieText.id) } >
                  <i className='bx bx-trash '></i>
                </button>
              </Flex>
            </Item>
            <Spacer margin='12px' />
          </>
        ))}
      </ul>
    </Container>
  )
}

export default App
