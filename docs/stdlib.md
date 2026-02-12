# Korlang Standard Library (Phase 1.3)

This document defines the initial standard library API surface for Phase 1.3, with a focus on IO, collections, concurrency, UI components, and data/AI pipelines.

## 1. I/O & Concurrency

### 1.1 Console
```
io.print(value: Any) -> Void
io.println(value: Any) -> Void
io.readLine() -> String?
```

### 1.2 File
```
io.File.open(path: String, mode: FileMode) -> Result<File, IOError>
io.File.readAll(self) -> Result<String, IOError>
io.File.readBytes(self) -> Result<[UInt], IOError>
io.File.write(self, data: String) -> Result<Void, IOError>
io.File.writeBytes(self, data: [UInt]) -> Result<Void, IOError>
io.File.close(self) -> Void
```

Types:
```
struct File { handle: Int }
enum FileMode { Read; Write; Append; ReadWrite; }
enum IOError { NotFound; PermissionDenied; InvalidPath; Unknown; }
```

### 1.3 Network
```
net.Socket.connect(addr: String, port: Int) -> Result<Socket, NetError>
net.Socket.listen(addr: String, port: Int) -> Result<Socket, NetError>
net.Socket.accept(self) -> Result<Socket, NetError>
net.Socket.send(self, data: [UInt]) -> Result<Int, NetError>
net.Socket.recv(self, max: Int) -> Result<[UInt], NetError>
net.Socket.close(self) -> Void
```

Types:
```
struct Socket { handle: Int }
enum NetError { ConnectionRefused; TimedOut; NotConnected; Unknown; }
```

### 1.4 Tasks and Channels
```
spawn task(f: fun() -> T) -> Task<T>
Task.await(self) -> Result<T, TaskError>
Task.cancel(self) -> Bool

Channel<T>.new(capacity: Int) -> Channel<T>
Channel.send(self, value: T) -> Result<Void, ChannelError>
Channel.recv(self) -> Result<T, ChannelError>
```

Types:
```
struct Task<T> { id: Int }
enum TaskError { Cancelled; Panicked; Unknown; }

struct Channel<T> { id: Int }
enum ChannelError { Closed; Full; Empty; }
```

## 2. Collections

### 2.1 List
```
List<T>.new() -> List<T>
List.push(self, value: T) -> Void
List.pop(self) -> T?
List.len(self) -> Int
List.get(self, idx: Int) -> T?
```

### 2.2 Map
```
Map<K, V>.new() -> Map<K, V>
Map.insert(self, key: K, value: V) -> V?
Map.get(self, key: K) -> V?
Map.remove(self, key: K) -> V?
Map.len(self) -> Int
```

### 2.3 Set
```
Set<T>.new() -> Set<T>
Set.insert(self, value: T) -> Bool
Set.contains(self, value: T) -> Bool
Set.remove(self, value: T) -> Bool
Set.len(self) -> Int
```

## 3. String and Regex

### 3.1 String
```
String.len(self) -> Int
String.isEmpty(self) -> Bool
String.slice(self, start: Int, end: Int) -> String
String.split(self, sep: String) -> [String]
String.trim(self) -> String
String.replace(self, from: String, to: String) -> String
```

### 3.2 Regex
```
Regex.new(pattern: String) -> Result<Regex, RegexError>
Regex.isMatch(self, s: String) -> Bool
Regex.find(self, s: String) -> Match?
Regex.findAll(self, s: String) -> [Match]
```

Types:
```
struct Regex { id: Int }
struct Match { start: Int, end: Int, text: String }
enum RegexError { InvalidPattern; Unknown; }
```

## 4. UI Component Library (Draft)

### 4.1 Core Components
```
VStack(spacing: Int = 0)
HStack(spacing: Int = 0)
Text(value: String)
Button(label: String, onClick: fun(Event) -> Void)
Image(src: String)
```

### 4.2 Common Properties
- `padding: Int`
- `margin: Int`
- `background: Color`
- `foreground: Color`
- `fontSize: Int`
- `fontWeight: Int`

### 4.3 Event Signature
```
struct Event { kind: String, targetId: String }
```

## 5. Data & AI Pipelines

### 5.1 Pipeline Primitives
```
Pipeline<T>.new(data: [T]) -> Pipeline<T>
Pipeline.map(self, f: fun(T) -> U) -> Pipeline<U>
Pipeline.filter(self, f: fun(T) -> Bool) -> Pipeline<T>
Pipeline.normalize(self, mean: Float, std: Float) -> Pipeline<Float>
Pipeline.collect(self) -> [T]
```

### 5.2 Tensor Interfaces
```
Tensor.load(path: String) -> Result<Tensor<Float, _>, IOError>
Tensor.inference(self, model: Model) -> Tensor<Float, _>
```

Types:
```
struct Model { id: Int }
```

