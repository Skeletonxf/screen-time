local totalTime = 0
local font = love.graphics.newFont(196)

function love.update(dt)
    totalTime = totalTime + dt
end

function love.draw()
    love.graphics.setFont(font)
    love.graphics.print(math.floor(totalTime), 50, 50)
end
