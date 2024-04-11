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
  Stack,
  Center,
  FormControl,
  FormLabel,
  ButtonGroup,
  Card,
  CardBody,
  CardFooter,
  CardHeader,

} from "@chakra-ui/react";

function App() {
  const [name, setName] = useState("");
  const [greetMsg, setGreetMsg] = useState("");
  const [file_path, setFilePath] = useState("");

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  async function play() {
    await invoke("play", { file_path });
  }

  async function pause() {
    await invoke("pause");
  }

  return (
    <>
      <Box bg="blue.200" p={6}>
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
      <Box bg="red.200" p={5} >
        <Center>
          <VStack>
            <Heading>Music Player</Heading>
            <HStack p={6} justifyContent="center">
              <Card >
                <CardBody>
                  <ButtonGroup>
                    <Button>Play</Button>
                    <Button>Pause</Button>
                  </ButtonGroup>
                </CardBody>
              </Card>
            </HStack>
            <FormControl>
              <FormLabel>Song Path</FormLabel>
              <Input placeholder="file path" value={file_path} onChange={e => setFilePath(e.target.value)}/>
            </FormControl>
          </VStack>
        </Center>
      </Box>
    </>
  );
}

export default App;
