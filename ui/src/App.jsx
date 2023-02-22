import { Button, Container, Flex, Input, Item, Spacer } from './styles'
import { useEffect, useState } from 'react'
import axios from 'axios'

function App() {
  const [movieText, setMovieText] = useState('')
  const [movieItem, setMovieItem] = useState([])
  const [movieDelete, setMovieDelete] = useState(false)

  useEffect(() => {
    axios
      .get('http://localhost:8000/blog-posts')
      .then((response) => {
        setMovieItem(response.data)
      })
      .catch((error) => {
        console.error('Failed to fetch data', error)
      })
  }, [movieDelete])

  const addMovie = (db) => {
    if (!movieText) return alert('Preencha um filme')
    const title = movieText.trim().toUpperCase()
    const newMovie = {
      title: title,
      body: 'Sample body',
      published: db == 'db1' ? true : false,
    }
    axios
      .post(`http://localhost:8000/blog-posts/${db}`, JSON.stringify(newMovie))
      .then((response) => {
        console.log('Success:', response.data)
        setMovieDelete(!movieDelete)
        setMovieText('')
      })
      .catch((error) => {
        console.error('Error:', error)
        return alert('Não é permitido itens duplicados')
        // Do something with the error, like show an error message
      })
  }

  const delteMovie = (db, title) => {
    axios
      .options(`http://localhost:8000/blog-posts/${db}/delete/${title}`)
      .then(() => {
        console.log('Movie deleted successfully')
        setMovieDelete(!movieDelete)
      })
      .catch((error) => {
        setMovieDelete(!movieDelete)
        console.error('Failed to delete movie', error)
      })
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
        <Button className='bx bx-data' id='bd1' onClick={() => addMovie('db1')}>
          BD1
        </Button>
        <Button className='bx bx-data' id='bd2' onClick={() => addMovie('db2')}>
          BD2
        </Button>
      </Flex>
      <Spacer margin='16px' />

      <ul>
        {movieItem?.map((movieText) => (
          // <React.Fragment key={movieText.title}>
          <>
            <Item
              // Item
              key={movieText.title}
              onClick={() =>
                console.log(movieText.id, movieText.body, movieText.published)
              }
            >
              <p>{movieText.title}</p>
              <Flex direction='row'>
                {movieText.body == 'both' ? (
                  <div>
                    <button onClick={() => delteMovie('db1', movieText.title)}>
                      <i className='bx bx-trash-alt '>1</i>
                    </button>
                    <button onClick={() => delteMovie('db2', movieText.title)}>
                      <i className='bx bx-trash '>2</i>
                    </button>
                  </div>
                ) : movieText.published ? (
                  <button onClick={() => delteMovie('db1', movieText.title)}>
                    <i className='bx bx-trash-alt '>1</i>
                  </button>
                ) : (
                  <button onClick={() => delteMovie('db2', movieText.title)}>
                    <i className='bx bx-trash '>2</i>
                  </button>
                )}
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
