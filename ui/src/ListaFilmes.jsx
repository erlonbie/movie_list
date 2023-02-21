import React, { useState, useEffect } from 'react'

// comment
function ListaFilmes() {

const [itemDb, setItemDb] = useState([])

useEffect(() => {
  fetch('http://localhost:8000/blog-posts/', { mode: 'no-cors' })
    .then((response) => response.json())
    .then((data) => {
      setItemDb(data)
      console.log(data)
    })
    .catch((error) => console.error(error))
}, [])

  return (

      // headers: {
      //   'Access-Control-Allow-Origin': '*',
      //   'Access-Control-Allow-Methods': 'POST, GET, PATCH, OPTIONS, DELETE',
      //   'Access-Control-Allow-Headers': '*',
      //   'Access-Control-Allow-Credentials': 'false',
      //   'Access-Control-Max-Age': 1200,
      // },
      // mode: 'no-cors',
      // crossdomain: true,
      // withCredentials: true,

      // <ul>
      //   {itemDb.map((textoFilme) => (
      //     <>
      //       <Item Item checked={textoFilme.checked} key={textoFilme.id}>
      //         <p>{textoFilme.textoFilme}</p>
      //         <Flex direction='row'>
      //           <button
      //             onClick={() =>
      //               toggleChecked(textoFilme.id, textoFilme.checked)
      //             }
      //           >
      //             <i class='bx bx-data '></i>
      //           </button>
      //           <button onClick={() => removeTask(textoFilme.id)}>
      //             <i class='bx bx-data '></i>
      //           </button>
      //         </Flex>
      //       </Item>
      //       <Spacer margin='12px' />
      //     </>
      //   ))}
      // </ul>
    <div>
      {itemDb.map(
        item => <div key={item.id}> {item.id} -- {item.title}</div>
      )}
    </div>
  )
}

export default ListaFilmes
