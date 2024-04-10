import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import {
  Box,
  Button,
  Input,
  Text,
  VStack,
  Heading,
  HStack,
  Center,
  FormControl,
  FormLabel,
  ButtonGroup,

} from "@chakra-ui/react";

function App() {
  const [name, setName] = useState("");
  const [greetMsg, setGreetMsg] = useState("");

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <>
      <Box p={6}>
        <Center>
          <VStack>
            <Heading>Welcome!</Heading>
            <FormControl>
              <FormLabel>Enter your name:</FormLabel>
              <Input placeholder="name" value={name} onChange={e => setName(e.target.value)}/>
              <ButtonGroup pt={3}>
                <Button onClick={greet}>Greet</Button>
                <Button onClick={() => {setName(""); setGreetMsg("");}}>Clear</Button>
              </ButtonGroup>
            </FormControl>
            <Text>{greetMsg}</Text>
          </VStack>
        </Center>
      </Box>
    </>
  );
}

export default App;
