import torch
from PIL import Image
import torchvision.transforms as transforms

# --- Load labels
with open("labels.txt", "r") as f:
    labels = [line.strip() for line in f.readlines()]

device = torch.device("cpu")

# --- Load model
model = torch.jit.load("model.pt", map_location=device)
model.eval()    
model = model.to(device)

# --- Preprocessing
transform = transforms.Compose([
    transforms.Resize((224, 224)),
    transforms.ToTensor(),
])

def classify(image_path):
    image = Image.open(image_path).convert("RGB")
    input_tensor = transform(image).unsqueeze(0).to(device)  # Add batch dimension
    with torch.no_grad():
        output = model(input_tensor)
        probs = torch.nn.functional.softmax(output[0], dim=0)
        top5 = torch.topk(probs, 5)
        for i in range(5):
            label = labels[top5.indices[i]]
            score = top5.values[i].item()
            print(f"{label}: {score:.2%}")

# --- Run
import sys
if len(sys.argv) != 2:
    print("Usage: python3 infer.py image.jpg")
else:
    classify(sys.argv[1])
