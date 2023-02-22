import styled, { createGlobalStyle } from 'styled-components'

export const GlobalStyle = createGlobalStyle`
  *{
    margin:0 ;
    padding: 0;
    box-sizing: border-box;
    text-decoration: none;
    outline: none;
    font-family: 'Poppins', sans-serif;
  }
`

export const Container = styled.div`
  width: 100%;
  min-height: 968px;
  background: rgba(0, 0, 0, 0.1);
  box-shadow: 0px 4px 4px rgba(0, 0, 0, 0.25);
  border-radius: 10px;

  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 12px 25px;

  .title {
    font-weight: 600;
    font-size: 48px;
    line-height: 72px;
    color: #8A6EB6;
  }
`

export const Input = styled.input`
  padding: 8px 24px;
  width: 324px;
  height: 50px;
  background: #ffffff;
  box-shadow: 0px 4px 4px rgba(0, 0, 0, 0.25);
  border-radius: 10px;
  border: none;

  font-weight: 700;
  font-size: 16px;
  line-height: 24px;
  color: #000000;

  &::placeholder {
    font-weight: 700;
    font-size: 16px;
    line-height: 24px;
    color: grey;
  }
`

export const Flex = styled.div`
  display: flex;
  flex-direction: ${(props) => props.direction || 'column'};
  justify-content: ${(props) => props.justify || 'center'};
  align-items: ${(props) => props.align || 'center'};
  gap: ${(props) => props.gap || '16px'};
`

export const Spacer = styled.div`
  width: 100%;
  margin: ${(props) => props.margin || '20px'};
`

export const Button = styled.button`
  width: 112px;
  height: 50px;
  background: ${(props) => (props.id == 'bd1' ? '#da2535' : '#5C97FF')};
  border-radius: 10px;
  border: none;

  text-align: center;
  align-items: center;
  display: flex;
  justify-content: center;
  /* padding: 10px; */

  font-weight: 500;
  font-size: 17px;
  line-height: 24px;
  color: #fbfbfb;
  box-shadow: 0px 4px 4px rgba(0, 0, 0, 0.25);

  cursor: pointer;

  &:hover {
    opacity: 0.8;
  }
  &:active {
    opacity: 0.6;
  }
`

export const Item = styled.li`
  padding: 13px 10px 13px 24px;
  width: 580px;
  min-height: 50px;
  background: ${(props) => (props.checked ? '#da2535' : '#FFFFFF')};
  box-shadow: 0px 4px 4px rgba(0, 0, 0, 0.25);
  border-radius: 10px;
  list-style: none;
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-wrap: wrap;

    &:hover {
      opacity: 0.85;
    }
    &:active {
      opacity: 0.5;
    }

  p {
    font-weight: 500;
    font-size: 16px;
    line-height: 24px;
    text-decoration-line: ${(props) => (props.checked ? 'line-through' : '')};
    /* color: ${(props) => (props.checked ? '#ffffff' : '#000000')}; */

    width: 80%;
    overflow-wrap: break-word;
    word-wrap: break-word;
    word-break: break-word;
  }

  button {
    background: transparent;
    /* color: ${props => props.color || "#da2535"}; */
    border: none;
    cursor: pointer;
    /* margin: ${props => props.size}; */
    margin: ${(props) => props.margin || '0px 13px 0px 0px'};

    &:hover {
      opacity: 0.3;
    }
    &:active {
      opacity: 0.6;
    }
  }

  i {
    font-size: 26px;
    text-align: center;
    align-items: center;
    display: flex;
    justify-content: center;
    /* color: ${props => props.color || "#da2535"}; */
    /* background: ${(props) => (props.id == 'bd1' ? 'red' : 'blue')}; */
    /* color: ${(props) => (props.checked ? '#ffffff' : '#000000')}; */
  }
`
